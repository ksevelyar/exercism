defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "AABBBCCCC" => "2A3B4C"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "2A3B4C" => "AABBBCCCC"
  """
  @spec encode(String.t()) :: String.t()
  def encode(string), do: do_encode(string, 1, nil, "")

  defp do_encode(<<>>, count, letter, acc) do
    count = if count > 1, do: count

    "#{acc}#{count}#{letter}"
  end
  defp do_encode(<<current_letter::binary-size(1), rest::binary>>, count, letter, acc)
       when current_letter == letter do
    do_encode(rest, count + 1, letter, acc)
  end
  defp do_encode(<<current_letter::binary-size(1), rest::binary>>, count, letter, acc) do
    count = if count > 1, do: count

    do_encode(rest, 1, current_letter, "#{acc}#{count}#{letter}")
  end

  @spec decode(String.t()) :: String.t()
  def decode(string), do: do_decode(string, 1, "")

  defp do_decode(<<>>, _count, acc), do: acc
  defp do_decode(string, count, acc) do
    case Integer.parse(string) do
      :error ->
        <<letter::binary-size(1)>> <> rest = string
        do_decode(rest, 1, acc <> String.duplicate(letter, count))

      {number, rest} ->
        do_decode(rest, number, acc)
    end
  end
end
