defmodule OcrNumbers do
  @lines_per_digit 4
  @cols_per_digit 3

  @doc """
  Given a 3 x 4 grid of pipes, underscores, and spaces, determine which number is represented, or
  whether it is garbled.
  """
  @spec convert([String.t()]) :: {:ok, String.t()} | {:error, String.t()}
  def convert(input) do
    with {:ok, input} <- validate_line_count(input),
         {:ok, input} <- validate_column_count(input) do
      lines =
        input |> Enum.chunk_every(@lines_per_digit) |> Enum.map(&convert_line/1) |> Enum.join(",")

      {:ok, lines}
    end
  end

  defp validate_line_count(input) do
    case rem(Enum.count(input), @lines_per_digit) do
      0 -> {:ok, input}
      _ -> {:error, "invalid line count"}
    end
  end

  defp validate_column_count([line | _rest] = input) do
    case rem(String.length(line), @cols_per_digit) do
      0 -> {:ok, input}
      _ -> {:error, "invalid column count"}
    end
  end

  defp convert_line(rows) do
    rows |> Enum.map(&split_row/1) |> Enum.zip() |> Enum.map(&convert_digit/1) |> Enum.join()
  end

  defp split_row(line) do
    for <<col::binary-@cols_per_digit <- line>>, do: col
  end

  defp convert_digit({
         " _ ",
         "| |",
         "|_|",
         "   "
       }),
       do: "0"

  defp convert_digit({
         "   ",
         "  |",
         "  |",
         "   "
       }),
       do: "1"

  defp convert_digit({
         " _ ",
         " _|",
         "|_ ",
         "   "
       }),
       do: "2"

  defp convert_digit({
         " _ ",
         " _|",
         " _|",
         "   "
       }),
       do: "3"

  defp convert_digit({
         "   ",
         "|_|",
         "  |",
         "   "
       }),
       do: "4"

  defp convert_digit({
         " _ ",
         "|_ ",
         " _|",
         "   "
       }),
       do: "5"

  defp convert_digit({
         " _ ",
         "|_ ",
         "|_|",
         "   "
       }),
       do: "6"

  defp convert_digit({
         " _ ",
         "  |",
         "  |",
         "   "
       }),
       do: "7"

  defp convert_digit({
         " _ ",
         "|_|",
         "|_|",
         "   "
       }),
       do: "8"

  defp convert_digit({
         " _ ",
         "|_|",
         " _|",
         "   "
       }),
       do: "9"

  defp convert_digit(_), do: "?"
end
