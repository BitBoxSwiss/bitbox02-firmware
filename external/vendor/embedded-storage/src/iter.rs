use crate::Region;

/// Iterator producing block-region pairs, where each memory block maps to each
/// region.
pub struct OverlapIterator<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	memory: &'a [u8],
	regions: I,
	base_address: u32,
}

/// Trait allowing us to automatically add an `overlaps` function to all iterators over [`Region`]
pub trait IterableByOverlaps<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	/// Obtain an [`OverlapIterator`] over a subslice of `memory` that overlaps with the region in `self`
	fn overlaps(self, memory: &'a [u8], base_address: u32) -> OverlapIterator<'a, R, I>;
}

impl<'a, R, I> Iterator for OverlapIterator<'a, R, I>
where
	R: Region,
	I: Iterator<Item = R>,
{
	type Item = (&'a [u8], R, u32);

	fn next(&mut self) -> Option<Self::Item> {
		for region in self.regions.by_ref() {
			//  TODO: This might be possible to do in a smarter way?
			let mut block_range = (0..self.memory.len())
				.skip_while(|index| !region.contains(self.base_address + *index as u32))
				.take_while(|index| region.contains(self.base_address + *index as u32));
			if let Some(start) = block_range.next() {
				let end = block_range.last().unwrap_or(start) + 1;
				return Some((
					&self.memory[start..end],
					region,
					self.base_address + start as u32,
				));
			}
		}
		None
	}
}

/// Blanket implementation for all types implementing [`Iterator`] over [`Region`]
impl<'a, R, I> IterableByOverlaps<'a, R, I> for I
where
	R: Region,
	I: Iterator<Item = R>,
{
	fn overlaps(self, memory: &'a [u8], base_address: u32) -> OverlapIterator<'a, R, I> {
		OverlapIterator {
			memory,
			regions: self,
			base_address,
		}
	}
}
