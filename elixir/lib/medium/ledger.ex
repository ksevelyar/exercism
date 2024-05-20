defmodule Ledger do
  @description_length 25
  @amount_length 14

  @doc """
  Format the given entries given a currency and locale
  """
  @type currency :: :usd | :eur
  @type locale :: :en_US | :nl_NL
  @type entry :: %{amount_in_cents: integer(), date: Date.t(), description: String.t()}

  @spec format_entries(currency(), locale(), list(entry())) :: String.t()
  def format_entries(currency, locale, entries) do
    entries =
      sort_entries(entries)
      |> Enum.map(fn entry -> format_entry(currency, locale, entry) end)
      |> Enum.join("")

    header(locale) <> entries
  end

  defp sort_entries(entries) do
    Enum.sort_by(entries, &{&1.date.day, &1.description, &1.amount_in_cents})
  end

  defp header(:nl_NL) do
    "Datum      | Omschrijving              | Verandering  \n"
  end

  defp header(_locale) do
    "Date       | Description               | Change       \n"
  end

  defp format_entry(currency, locale, entry) do
    year = entry.date.year |> to_string()
    month = entry.date.month |> to_string() |> String.pad_leading(2, "0")
    day = entry.date.day |> to_string() |> String.pad_leading(2, "0")
    date = date(locale, day, month, year)

    decimal =
      entry.amount_in_cents |> abs() |> rem(100) |> to_string() |> String.pad_leading(2, "0")

    whole = entry.amount_in_cents |> abs() |> div(100) |> whole(locale)
    number = number(locale, decimal, whole)

    currency = if currency == :eur, do: "€", else: "$"

    amount =
      locale |> amount(currency, entry.amount_in_cents, number) |> String.pad_leading(@amount_length, " ")

    description = description(entry.description)

    date <> "|" <> description <> " |" <> amount <> "\n"
  end

  defp date(:nl_NL, day, month, year) do
    day <> "-" <> month <> "-" <> year <> " "
  end

  defp date(_locale, day, month, year) do
    month <> "/" <> day <> "/" <> year <> " "
  end

  defp whole(whole, :nl_NL) when whole > 1000 do
    "#{div(whole, 1000)}.#{rem(whole, 1000)}"
  end

  defp whole(whole, _locale) when whole > 1000 do
    "#{div(whole, 1000)},#{rem(whole, 1000)}"
  end

  defp whole(whole, _locale), do: whole

  defp number(:nl_NL, decimal, whole) do
    "#{whole},#{decimal}"
  end

  defp number(_locale, decimal, whole) do
    "#{whole}.#{decimal}"
  end

  defp amount(:nl_NL, currency, amount_in_cents, number) when amount_in_cents >= 0 do
    " #{currency} #{number} "
  end

  defp amount(:nl_NL, currency, _amount_in_cents, number) do
    " #{currency} -#{number} "
  end

  defp amount(_locale, currency, amount_in_cents, number) when amount_in_cents >= 0 do
    "  #{currency}#{number} "
  end

  defp amount(_locale, currency, _amount_in_cents, number) do
    " (#{currency}#{number})"
  end

  defp description(description) do
    if String.length(description) <= @description_length do
      " " <> String.pad_trailing(description, @description_length, " ")
    else
      " " <> String.slice(description, 0, @description_length - 3) <> "..."
    end
  end
end
