defmodule NucleotideCount do
  @nucleotides [?A, ?C, ?G, ?T]

  @doc """
  Counts individual nucleotides in a DNA strand.

  ## Examples

  iex> NucleotideCount.count('AATAA', ?A)
  4

  iex> NucleotideCount.count('AATAA', ?T)
  1
  """
  def count(strand, nucleotide) do
    strand |> Enum.filter(&(&1 == nucleotide)) |> Enum.count()
  end

  @doc """
  Returns a summary of counts by nucleotide.

  ## Examples

  iex> NucleotideCount.histogram('AATAA')
  %{?A => 4, ?T => 1, ?C => 0, ?G => 0}
  """
  def histogram(strand) do
    Enum.reduce(strand, %{?A => 0, ?T => 0, ?C => 0, ?G => 0}, fn nucleotide, acc ->
      %{acc | nucleotide => acc[nucleotide] + 1}
    end)
  end
end
