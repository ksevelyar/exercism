defmodule AffineCipher do
  @alphabet_length 26
  @encoded_word_length 5
  @digits ?0..?9

  def encode(%{a: a, b: b}, message) do
    if coprime?(a) do
      {:ok, do_encode(a, b, message)}
    else
      {:error, "a and m must be coprime."}
    end
  end

  defp do_encode(a, b, message) do
    message
    |> normalize()
    |> String.to_charlist()
    |> Stream.map(&encode_char(&1, a, b))
    |> Enum.chunk_every(@encoded_word_length)
    |> Enum.join(" ")
  end

  defp coprime?(a) do
    Integer.gcd(a, @alphabet_length) == 1
  end

  defp normalize(str) do
    str
    |> String.downcase()
    |> String.replace(~r/[^\d\w]/, "")
  end

  defp encode_char(ch, _a, _b) when ch in @digits, do: ch

  defp encode_char(ch, a, b) do
    index = ch - ?a
    shift = rem(a * index + b, @alphabet_length)

    ?a + shift
  end

  def decode(%{a: a, b: b}, encrypted) do
    if coprime?(a) do
      {:ok, do_decode(a, b, encrypted)}
    else
      {:error, "a and m must be coprime."}
    end
  end

  defp do_decode(a, b, message) do
    mmi = modular_multiplicative_inverse(a)

    message
    |> normalize()
    |> String.to_charlist()
    |> Enum.map(&decode_char(&1, mmi, b))
    |> List.to_string()
  end

  defp modular_multiplicative_inverse(a) do
    Enum.find(1..@alphabet_length, fn n -> rem(n * a, @alphabet_length) == 1 end)
  end

  defp decode_char(ch, _mmi, _b) when ch in @digits, do: ch

  defp decode_char(ch, mmi, b) do
    index = ch - ?a
    shift = Integer.mod(mmi * (index - b), 26)

    ?a + shift
  end
end
