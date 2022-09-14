defmodule Hamming do
  @doc """
  Returns number of differences between two strands of DNA, known as the Hamming Distance.

  ## Examples

  iex> Hamming.hamming_distance('AAGTCATA', 'TAGCGATC')
  {:ok, 4}
  """

  def hamming_distance(strand1, strand2) when length(strand1) == length(strand2) do
    {:ok, Enum.zip(strand1, strand2) |> Enum.count(fn {ch1, ch2} -> ch1 != ch2 end)}
  end

  def hamming_distance(_strand1, _strand2), do: {:error, "strands must be of equal length"}
end
