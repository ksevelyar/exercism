defmodule RailFenceCipher do
  @doc """
  Encode a given plaintext to the corresponding rail fence ciphertext
  """
  @spec encode(String.t(), pos_integer) :: String.t()
  def encode(str, 1), do: str

  def encode(str, rails) do
    chars_on_rails = Enum.zip(sequence(rails), String.codepoints(str))

    Enum.flat_map(1..rails, fn rail ->
      Enum.filter(chars_on_rails, fn {ch_rail, _ch} -> rail == ch_rail end)
    end)
    |> Enum.map(fn {_rail, ch} -> ch end)
    |> Enum.join()
  end

  @doc """
  Decode a given rail fence ciphertext to the corresponding plaintext
  """
  @spec decode(String.t(), pos_integer) :: String.t()
  def decode(str, 1), do: str

  def decode(str, rails) do
    length = String.length(str)
    sequence = rails |> sequence() |> Enum.take(length)

    chars_on_rails =
      Enum.zip(String.codepoints(str), Enum.sort(sequence))
      |> Enum.group_by(&elem(&1, 1), &elem(&1, 0))

    Enum.reduce(sequence, [], fn rail_index, acc ->
      rail = chars_on_rails[rail_index]
      char_index = Enum.count(acc, fn {i, _} -> i == rail_index end)
      char = Enum.at(rail, char_index)

      [{rail_index, char} | acc]
    end)
    |> Enum.map(fn {_, char} -> char end)
    |> Enum.reverse()
    |> Enum.join()
  end

  defp sequence(rails) do
    Stream.cycle(Enum.to_list(1..rails) ++ Enum.to_list((rails - 1)..2))
  end
end
