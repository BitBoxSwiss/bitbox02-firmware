// SPDX-License-Identifier: Apache-2.0
//
// Generates EIP1559 test vectors as JSON for sighash tests.
// Usage: go run gen_eip1559_tests.go > eip1559_tests.json

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
	ChainID           uint64 `json:"chain_id"`
	Nonce             string `json:"nonce"`
	MaxPriorityFee    string `json:"max_priority_fee"`
	MaxFeePerGas      string `json:"max_fee_per_gas"`
	GasLimit          string `json:"gas_limit"`
	Recipient         string `json:"recipient"`
	Value             string `json:"value"`
	Data              string `json:"data"`
	ExpectedSighash   string `json:"expected_sighash"`
}

var nets = []*params.ChainConfig{
	params.MainnetChainConfig,
	params.SepoliaChainConfig,
	{
		ChainID:     big.NewInt(200),
		LondonBlock: big.NewInt(12965000),
		CancunTime:  params.MainnetChainConfig.CancunTime,
	},
	{
		ChainID:     big.NewInt(9223372036854775803),
		LondonBlock: big.NewInt(12965000),
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

func main() {
	rand.Seed(42)

	var tests []TestCase

	for i := 0; i < 100; i++ {
		nonce := uint64(rand.Intn(10000))
		maxFeePerGas := new(big.Int).SetBytes(randbytes(rand.Intn(16) + 1))
		maxPriorityFeePerGas := new(big.Int).Rand(rand.New(rand.NewSource(42)), maxFeePerGas)
		gasLimit := rand.Uint64()
		recBytes := randbytes(20)
		recipient := common.BytesToAddress(recBytes)
		amount := new(big.Int).SetBytes(randbytes(rand.Intn(16) + 1))
		data := randbytes(rand.Intn(1025))
		accessList := types.AccessList{}

		net := nets[rand.Intn(len(nets))]

		txData := &types.DynamicFeeTx{
			Nonce:      nonce,
			GasTipCap:  maxPriorityFeePerGas,
			GasFeeCap:  maxFeePerGas,
			Gas:        gasLimit,
			To:         &recipient,
			Value:      amount,
			Data:       data,
			AccessList: accessList,
		}
		tx := types.NewTx(txData)

		signer := types.MakeSigner(net, net.LondonBlock, *net.CancunTime)
		sighash := signer.Hash(tx)

		test := TestCase{
			ChainID:         uint64(net.ChainID.Int64()),
			Nonce:           encbytes(new(big.Int).SetUint64(tx.Nonce()).Bytes()),
			MaxPriorityFee:  encbytes(tx.GasTipCap().Bytes()),
			MaxFeePerGas:    encbytes(tx.GasFeeCap().Bytes()),
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
