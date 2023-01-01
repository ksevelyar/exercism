defmodule Atbash do
  @encode_word_length 5
  @map Enum.zip(?a..?z, ?z..?a) |> Map.new()

  def decode(cipher) do
    chars =
      cipher
      |> String.downcase()
      |> String.to_charlist()
      |> Enum.filter(&(&1 != ?\s))

    chars |> Enum.map(&(@map[&1] || &1)) |> Enum.reject(&is_nil/1) |> to_string()
  end

  def encode(plaintext) do
    chars =
      plaintext
      |> String.downcase()
      |> String.to_charlist()
      |> Enum.filter(&(Enum.member?(?a..?z, &1) || Enum.member?(?0..?9, &1)))
      |> Enum.with_index()

    for({ch, index} <- chars, !is_nil(ch), do: encode_char(ch, index)) |> to_string()
  end

  defp encode_char(ch, index) do
    maybe_encoded_char = @map[ch] || ch
    insert_space? = index > 0 and rem(index, @encode_word_length) == 0

    case insert_space? do
      true -> [' ', maybe_encoded_char]
      false -> maybe_encoded_char
    end
  end
end
