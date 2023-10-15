defmodule PhoneNumber do
  @doc """
  Remove formatting from a phone number if the given number is valid. Return an error otherwise.
  """
  @spec clean(String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def clean(raw) do
    digits = remove_separators(raw)

    with :ok <- validate_non_digits(digits),
         {:ok, digits} <- validate_country_code(digits),
         :ok <- validate_length(digits),
         :ok <- validate_area(digits),
         :ok <- validate_exchange(digits) do
      {:ok, digits}
    end
  end

  defp remove_separators(input) do
    String.replace(input, ~r/[-.\(\)\s+]/, "")
  end

  defp validate_length(string) do
    case String.length(string) do
      len when len > 11 -> {:error, "must not be greater than 11 digits"}
      len when len < 10 -> {:error, "must not be fewer than 10 digits"}
      _ -> :ok
    end
  end

  defp validate_country_code(digits) do
    case {String.length(digits), String.starts_with?(digits, "1")} do
      {11, true} -> {:ok, String.slice(digits, 1..-1)}
      {11, false} -> {:error, "11 digits must start with 1"}
      _ -> {:ok, digits}
    end
  end

  defp validate_non_digits(input) do
    if String.match?(input, ~r/^\d+$/) do
      :ok
    else
      {:error, "must contain digits only"}
    end
  end

  defp validate_area("0" <> _), do: {:error, "area code cannot start with zero"}
  defp validate_area("1" <> _), do: {:error, "area code cannot start with one"}
  defp validate_area(_), do: :ok

  defp validate_exchange(<<_area::binary-size(3)>> <> "0" <> _), do: {:error, "exchange code cannot start with zero"}
  defp validate_exchange(<<_area::binary-size(3)>> <> "1" <> _), do: {:error, "exchange code cannot start with one"}
  defp validate_exchange(_), do: :ok
end
