defmodule Transpose do
  @doc """
  Given an input text, output it transposed.

  Rows become columns and columns become rows. See https://en.wikipedia.org/wiki/Transpose.

  If the input has rows of different lengths, this is to be solved as follows:
    * Pad to the left with spaces.
    * Don't pad to the right.

  ## Examples

    iex> Transpose.transpose("ABC\\nDE")
    "AD\\nBE\\nC"

    iex> Transpose.transpose("AB\\nDEF")
    "AD\\nBE\\n F"
  """

  @spec transpose(String.t()) :: String.t()
  def transpose(input) do
    lines = input |> String.split("\n") |> Enum.map(&String.graphemes/1)

    longest_line_length = Enum.max_by(lines, &length(&1)) |> length()

    # lines = Enum.map(lines, fn line -> pad_line(line, longest_line_length) end)

    0..(longest_line_length - 1)
    |> Enum.map(fn row ->
      lines
      |> Enum.map(fn line -> Enum.at(line, row) end)
      |> Enum.join()
    end)
    |> Enum.join("\n")
  end

  defp pad_line(line, longest_line_length) do
    line ++ List.duplicate(" ", longest_line_length - length(line))
  end
end
