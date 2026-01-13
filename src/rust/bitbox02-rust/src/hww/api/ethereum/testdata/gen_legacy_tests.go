// SPDX-License-Identifier: Apache-2.0
//
// Generates Legacy transaction test vectors as JSON for sighash tests.
// Usage: go run gen_legacy_tests.go > legacy_tests.json

package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"math/rand"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
)

type TestCase struct {
	ChainID         uint64 `json:"chain_id"`
	Nonce           string `json:"nonce"`
	GasPrice        string `json:"gas_price"`
	GasLimit        string `json:"gas_limit"`
	Recipient       string `json:"recipient"`
	Value           string `json:"value"`
	Data            string `json:"data"`
	ExpectedSighash string `json:"expected_sighash"`
}

var nets = []*params.ChainConfig{
	params.MainnetChainConfig,
	params.SepoliaChainConfig,
	{
		ChainID:     big.NewInt(200),
		EIP155Block: big.NewInt(0),
		CancunTime:  params.MainnetChainConfig.CancunTime,
	},
	{
		ChainID:     big.NewInt(9223372036854775803),
		EIP155Block: big.NewInt(0),
		CancunTime:  params.MainnetChainConfig.CancunTime,
	},
}

func encbytes(bs []byte) string {
	return hex.EncodeToString(bs)
}

func randbytes(n int) []byte {
	res := make([]byte, n)
	nn, err := rand.Read(res)
	if err != nil || nn != n {
		panic("randbytes")
	}
	return res
}

func generateDataSize(i int) int {
	// 80% small (0-1024 bytes), 20% large (5KB, 8KB, 12KB, 15KB)
	if i%5 == 0 {
		// Large data sizes for testing multi-chunk streaming
		largeSizes := []int{5 * 1024, 8 * 1024, 12 * 1024, 15 * 1024}
		return largeSizes[rand.Intn(len(largeSizes))]
	}
	return rand.Intn(1025)
}

func main() {
	rand.Seed(42)

	var tests []TestCase

	for i := 0; i < 200; i++ {
		recipient := randbytes(20)
		nonce := uint64(rand.Intn(10000))
		amount := new(big.Int).SetBytes(randbytes(rand.Intn(16) + 1))

		gasLimit := rand.Uint64()
		gasPrice := new(big.Int).SetBytes(randbytes(rand.Intn(16) + 1))
		data := randbytes(generateDataSize(i))

		net := nets[rand.Intn(len(nets))]

		tx := types.NewTransaction(
			nonce, common.BytesToAddress(recipient), amount, gasLimit,
			gasPrice, data)

		signer := types.MakeSigner(net, net.EIP155Block, *net.CancunTime)
		sighash := signer.Hash(tx)

		test := TestCase{
			ChainID:         uint64(net.ChainID.Int64()),
			Nonce:           encbytes(new(big.Int).SetUint64(tx.Nonce()).Bytes()),
			GasPrice:        encbytes(tx.GasPrice().Bytes()),
			GasLimit:        encbytes(new(big.Int).SetUint64(tx.Gas()).Bytes()),
			Recipient:       encbytes(tx.To().Bytes()),
			Value:           encbytes(tx.Value().Bytes()),
			Data:            encbytes(tx.Data()),
			ExpectedSighash: encbytes(sighash.Bytes()),
		}
		tests = append(tests, test)
	}

	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(tests); err != nil {
		fmt.Fprintf(os.Stderr, "Error encoding JSON: %v\n", err)
		os.Exit(1)
	}
}
