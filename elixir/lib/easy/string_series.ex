defmodule StringSeries do
  @doc """
  Given a string `str` and a positive integer `size`, return all substrings
  of that size. If `size` is greater than the length of `str`, or less than 1,
  return an empty list.
  """
  @spec slices(str :: String.t(), size :: integer) :: list(String.t())
  def slices(str, size), do: do_slices(String.length(str), str, size)

  defp do_slices(length, _str, size) when size <= 0 or size > length, do: []
  defp do_slices(_length, str, size) do
    str
    |> String.graphemes()
    |> Stream.chunk_every(size, 1, :discard)
    |> Enum.map(&Enum.join/1)
  end
end
