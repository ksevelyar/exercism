defmodule Markdown do
  @doc """
    Parses a given string with Markdown syntax and returns the associated HTML for that string.

    ## Examples

      iex> Markdown.parse("This is a paragraph")
      "<p>This is a paragraph</p>"

      iex> Markdown.parse("# Header!\\n* __Bold Item__\\n* _Italic Item_")
      "<h1>Header!</h1><ul><li><strong>Bold Item</strong></li><li><em>Italic Item</em></li></ul>"
  """
  @spec parse(String.t()) :: String.t()
  def parse(string) do
    string
    |> String.split("\n")
    |> Enum.map(&process_line/1)
    |> Enum.join()
    |> enclose_with_ul_tag()
  end

  defp process_line(line) do
    cond do
      String.starts_with?(line, "#") && !String.starts_with?(line, "#######") ->
        enclose_with_header_tag(line)

      String.starts_with?(line, "*") ->
        parse_list_md_level(line)

      true ->
        enclose_with_paragraph_tag(line)
    end
  end

  defp parse_list_md_level(line) do
    line = String.trim_leading(line, "* ")

    "<li>#{enclose_with_bold_and_italic(line)}</li>"
  end

  defp enclose_with_header_tag(header) do
    [header_level, header_text] = String.split(header, " ", parts: 2)
    header_level = String.length(header_level)

    "<h#{header_level}>#{header_text}</h#{header_level}>"
  end

  defp enclose_with_paragraph_tag(line) do
    "<p>#{enclose_with_bold_and_italic(line)}</p>"
  end

  defp enclose_with_bold_and_italic(line) do
    line
    |> String.replace(~r/__(.+?)__/, "<strong>\\1</strong>")
    |> String.replace(~r/_(.+?)_/, "<em>\\1</em>")
  end

  defp enclose_with_ul_tag(text) do
    String.replace(text, ~r/<li>.+<\/li>/, "<ul>\\0</ul>")
  end
end
