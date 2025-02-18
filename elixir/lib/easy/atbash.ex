defmodule Atbash do
  @encode_word_length 5
  @map Enum.zip(?a..?z, ?z..?a//-1) |> Map.new()
  @allowed_chars Stream.concat(?a..?z, ?0..?9) |> Enum.to_list()

  def decode(cipher) do
    filter_spaces = fn charlist -> Enum.reject(charlist, &(&1 == ?\s)) end

    chars =
      cipher
      |> String.downcase()
      |> String.to_charlist()
      |> filter_spaces.()

    chars |> Enum.map(&(@map[&1] || &1)) |> Enum.reject(&is_nil/1) |> to_string()
  end

  def encode(plaintext) do
    filter_not_allowed_chars = fn charlist ->
      Enum.filter(charlist, &Enum.member?(@allowed_chars, &1))
    end

    chars =
      plaintext
      |> String.downcase()
      |> String.to_charlist()
      |> filter_not_allowed_chars.()
      |> Enum.with_index()

    chars |> Enum.map(&encode_char/1) |> to_string()
  end

  defp encode_char({ch, index}) do
    maybe_encoded_char = @map[ch] || ch
    insert_space? = index > 0 and rem(index, @encode_word_length) == 0

    case insert_space? do
      true -> [~c" ", maybe_encoded_char]
      false -> maybe_encoded_char
    end
  end
end
