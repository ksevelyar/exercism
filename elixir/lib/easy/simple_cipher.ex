defmodule SimpleCipher do
  @chars ?a..?z

  def encode(plaintext, key) do
    rotate(plaintext, key, &encode_char/2)
  end

  def decode(ciphertext, key) do
    rotate(ciphertext, key, &decode_char/2)
  end

  def generate_key(length) do
    1..length |> Enum.map(fn _ -> Enum.random(@chars) end) |> List.to_string()
  end

  defp rotate(message, key, rotate_char_fun) do
    key = key |> String.to_charlist() |> Stream.cycle()

    message
    |> String.to_charlist()
    |> Enum.zip(key)
    |> Enum.map(fn {ch, key_ch} -> rotate_char_fun.(ch, key_ch) end)
    |> List.to_string()
  end

  defp decode_char(ch, key_ch) do
    ?a + Integer.mod(ch - key_ch, 26)
  end

  defp encode_char(ch, key_ch) do
    ?a + rem(ch - ?a + key_ch - ?a, 26)
  end
end
