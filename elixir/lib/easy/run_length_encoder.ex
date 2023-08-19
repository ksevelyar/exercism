defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "AABBBCCCC" => "2A3B4C"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "2A3B4C" => "AABBBCCCC"
  """
  @spec encode(String.t()) :: String.t()
  def encode(string) do
    do_encode(string, 0, nil, "")
  end

  defp do_encode(<<>>, count, letter, acc) do
    count = if count == 1 or letter == nil, do: "", else: count

    "#{acc}#{count}#{letter}"
  end

  defp do_encode(<<current_letter::bitstring-size(8), rest::binary>>, count, letter, acc)
       when current_letter == letter do
    do_encode(rest, count + 1, letter, acc)
  end

  defp do_encode(<<current_letter::bitstring-size(8), rest::binary>>, 0, nil, acc) do
    do_encode(rest, 1, current_letter, acc)
  end

  defp do_encode(<<current_letter::bitstring-size(8), rest::binary>>, count, letter, acc) do
    count = if count == 1, do: "", else: count

    do_encode(rest, 1, current_letter, "#{acc}#{count}#{letter}")
  end

  @spec decode(String.t()) :: String.t()
  def decode(string) do
  end
end
