defmodule Atbash do
  @map Enum.zip(?a..?z, ?z..?a) |> Map.new()

  def encode(plaintext) do
    chars =
      plaintext
      |> String.downcase()
      |> String.to_charlist()
      |> Enum.filter(&(Enum.member?(?a..?z, &1) || Enum.member?(?0..?9, &1)))
      |> Enum.with_index()

    for({ch, index} <- chars, !is_nil(ch), do: encode_char(ch, index)) |> to_string()
  end

  def decode(cipher) do
    chars =
      cipher
      |> String.downcase()
      |> String.to_charlist()
      |> Enum.filter(&(&1 != ?\s))

    chars |> Enum.map(&decode_char/1) |> Enum.reject(&is_nil/1) |> to_string()
  end

  defp encode_char(ch, index) do
    encoded_char = @map[ch] || ch

    case rem(index, 5) do
      0 when index > 0 -> [' ', encoded_char]
      _ -> encoded_char
    end
  end

  defp decode_char(ch) do
    @map[ch] || ch
  end
end
