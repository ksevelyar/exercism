defmodule CryptoSquare do
  @doc """
  Encode string square methods
  ## Examples

    iex> CryptoSquare.encode("abcd")
    "ac bd"
  """
  @spec encode(String.t()) :: String.t()
  def encode(str) do
    str |> normalize() |> maybe_encode()
  end

  defp maybe_encode(""), do: ""

  defp maybe_encode(str) do
    row_length = str |> String.length() |> :math.sqrt() |> ceil()

    rows = split_every(str, row_length)

    0..(row_length - 1)
    |> Enum.map(fn index ->
      rows |> Enum.map(fn row -> String.at(row, index) || " " end) |> Enum.join()
    end)
    |> Enum.join(" ")
  end

  defp split_every(str, size) do
    str
    |> String.graphemes()
    |> Stream.chunk_every(size)
    |> Enum.map(&Enum.join/1)
  end

  defp normalize(str) do
    str
    |> String.downcase()
    |> String.replace(~r/[^\d\w]/, "")
  end
end
