defmodule Grep do
  @line_numbers "-n"
  @only_file_names "-l"
  @case_insensitive "-i"
  @exact_match "-x"
  @inverse_match "-v"

  @spec grep(String.t(), [String.t()], [String.t()]) :: String.t()
  def grep(pattern, flags, files) do
    Enum.map(files, fn file -> grep_file(file, pattern, flags, length(files) > 1) end)
    |> List.flatten()
    |> Enum.join("")
  end

  defp grep_file(file, pattern, flags, multiple_files?) do
    file
    |> File.stream!()
    |> Stream.with_index()
    |> Stream.filter(fn {line, _ind} -> line_matches?(flags, line, pattern) end)
    |> maybe_add_line_numbers(@line_numbers in flags and @only_file_names not in flags)
    |> maybe_prepend_with_file_name(file, multiple_files? and @only_file_names not in flags)
    |> maybe_replace_matches_with_file_name(file, @only_file_names in flags)
  end

  defp maybe_replace_matches_with_file_name([], _file, _), do: []
  defp maybe_replace_matches_with_file_name(_lines, file, true), do: ["#{file}\n"]
  defp maybe_replace_matches_with_file_name(lines, _file, false), do: lines

  defp maybe_add_line_numbers(lines, true) do
    Enum.map(lines, fn {line, ind} -> "#{ind + 1}:#{line}" end)
  end

  defp maybe_add_line_numbers(lines, false) do
    Enum.map(lines, fn {line, _ind} -> line end)
  end

  defp maybe_prepend_with_file_name(lines, file, true) do
    Enum.map(lines, fn line -> "#{file}:#{line}" end)
  end

  defp maybe_prepend_with_file_name(lines, _file, false), do: lines

  defp line_matches?(flags, line, pattern) do
    {line, pattern} =
      cond do
        @case_insensitive in flags -> {String.downcase(line), String.downcase(pattern)}
        true -> {line, pattern}
      end

    matches? =
      cond do
        @exact_match in flags -> String.trim(line) == pattern
        true -> String.contains?(line, pattern)
      end

    cond do
      @inverse_match in flags -> !matches?
      true -> matches?
    end
  end
end
