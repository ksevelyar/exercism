defmodule ZebraPuzzle do
  @statements [
    {{:position, 1}, :same, {:nationality, :norwegian}},
    {{:position, 3}, :same, {:drink, :milk}},
    {{:nationality, :norwegian}, :next, {:color, :blue}},
    {{:color, :red}, :same, {:nationality, :englishman}},
    {{:nationality, :ukrainian}, :same, {:drink, :tea}},
    {{:color, :green}, :same, {:drink, :coffee}},
    {{:drink, :orange_juice}, :same, {:hobby, :football}},
    {{:nationality, :japanese}, :same, {:hobby, :chess}},
    {{:nationality, :spaniard}, :same, {:pet, :dog}},
    {{:pet, :snail}, :same, {:hobby, :dancer}},
    {{:pet, :fox}, :next, {:hobby, :reader}},
    {{:color, :green}, :next_to_the_right, {:color, :ivory}},
    {{:color, :yellow}, :same, {:hobby, :painter}},
    {{:hobby, :painter}, :next, {:pet, :horse}}
  ]

  @initial_state [
    %{
      color: [:blue, :red, :ivory, :green, :yellow],
      position: [1],
      nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
      pet: [:snail, :fox, :horse, :dog, :zebra],
      drink: [:tea, :water, :milk, :coffee, :orange_juice],
      hobby: [:chess, :reader, :painter, :football, :dancer]
    },
    %{
      color: [:blue, :red, :ivory, :green, :yellow],
      position: [2],
      nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
      pet: [:snail, :fox, :horse, :dog, :zebra],
      drink: [:tea, :water, :milk, :coffee, :orange_juice],
      hobby: [:chess, :reader, :painter, :football, :dancer]
    },
    %{
      color: [:blue, :red, :ivory, :green, :yellow],
      position: [3],
      nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
      pet: [:snail, :fox, :horse, :dog, :zebra],
      drink: [:tea, :water, :milk, :coffee, :orange_juice],
      hobby: [:chess, :reader, :painter, :football, :dancer]
    },
    %{
      color: [:blue, :red, :ivory, :green, :yellow],
      position: [4],
      nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
      pet: [:snail, :fox, :horse, :dog, :zebra],
      drink: [:tea, :water, :milk, :coffee, :orange_juice],
      hobby: [:chess, :reader, :painter, :football, :dancer]
    },
    %{
      color: [:blue, :red, :ivory, :green, :yellow],
      position: [5],
      nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
      pet: [:snail, :fox, :horse, :dog, :zebra],
      drink: [:tea, :water, :milk, :coffee, :orange_juice],
      hobby: [:chess, :reader, :painter, :football, :dancer]
    }
  ]

  @properties %{
    drink: [:tea, :water, :milk, :coffee, :orange_juice],
    color: [:blue, :red, :ivory, :green, :yellow],
    hobby: [:chess, :reader, :painter, :football, :dancer],
    nationality: [:norwegian, :ukrainian, :japanese, :spaniard, :englishman],
    pet: [:snail, :fox, :horse, :dog, :zebra]
  }

  defp invalid_house?({{key1, value1}, :same, {key2, value2}}, house, _houses) do
    value1 not in house[key1] or value2 not in house[key2]
  end

  defp invalid_house?({{key1, value1}, :next, {key2, value2}}, house, houses) do
    invalid_house? = value1 not in house[key1] and value2 not in house[key2]

    condition_houses =
      Enum.filter(houses, fn condition_house ->
        [position] = house.position

        next =
          condition_house.position == [position + 1] || condition_house.position == [position - 1]

        valid_neighbour =
          if value1 in house[key1] do
            value2 in condition_house[key2]
          else
            value1 in condition_house[key1]
          end

        next && valid_neighbour
      end)

    Enum.empty?(condition_houses) || invalid_house?
  end

  defp invalid_house?({{key1, value1}, :next_to_the_right, {key2, value2}}, house, houses) do
    invalid_house? = value1 not in house[key1] and value2 not in house[key2]
    [position] = house.position

    condition_houses =
      Enum.filter(houses, fn condition_house ->
        (value2 in condition_house[key2] and condition_house.position == [position - 1] &&
           value1 in house[key1]) ||
          (value1 in condition_house[key1] and condition_house.position == [position + 1] &&
             value2 in house[key2])
      end)

    Enum.empty?(condition_houses) || invalid_house?
  end

  defp valid_house?({{key1, value1}, :next, {key2, value2}}, house, houses) do
    [position] = house.position

    Enum.any?(houses, fn condition_house ->
      valid_position =
        condition_house.position == [position + 1] || condition_house.position == [position - 1]

      single_match =
        Enum.filter(houses, fn c_house ->
          [cpos] = condition_house.position

          valid_position = c_house.position == [cpos + 1] || c_house.position == [cpos - 1]

          match =
            (condition_house[key1] == [value1] and value2 in c_house[key2]) or
              (condition_house[key2] == [value2] and value1 in c_house[key1])

          valid_position && match
        end)

      valid_position && single_match == [house]
    end)
  end

  defp valid_house?({{key1, value1}, :next_to_the_right, {key2, value2}}, house, houses) do
    Enum.any?(houses, fn condition_house ->
      [position] = house.position

      next =
        condition_house.position == [position + 1]

      valid_neighbour =
        condition_house[key2] == [value2] and value1 in house[key1]

      next && valid_neighbour
    end)
  end

  defp valid_house?({{key1, value1}, :same, {key2, value2}}, house, _houses) do
    house[key1] == [value1] || house[key2] == [value2]
  end

  defp set_properties({{key1, value1}, :same, {key2, value2}}, house) do
    Map.put(house, key1, [value1])
    |> Map.put(key2, [value2])
  end

  defp set_properties({{key1, value1}, :next, {key2, value2}}, house) do
    if value1 in house[key1] do
      Map.put(house, key1, [value1])
      |> Map.update(key2, [], fn list -> list -- [value2] end)
    else
      Map.put(house, key2, [value2])
      |> Map.update(key1, [], fn list -> list -- [value1] end)
    end
  end

  defp set_properties({{key1, value1}, :next_to_the_right, {key2, value2}}, house) do
    if value1 in house[key1] do
      Map.put(house, key1, [value1])
      |> Map.update(key2, [], fn list -> list -- [value2] end)
    else
      Map.put(house, key2, [value2])
      |> Map.update(key1, [], fn list -> list -- [value1] end)
    end
  end

  defp remove_properties({{key1, value1}, :same, {key2, value2}}, house) do
    if value1 in house[key1] do
      Map.update(house, key1, [], fn list -> list -- [value1] end)
    else
      Map.update(house, key2, [], fn list -> list -- [value2] end)
    end
  end

  defp remove_properties({{key1, value1}, :next, {key2, value2}}, house) do
    if value1 in house[key1] do
      Map.update(house, key1, [], fn list -> list -- [value1] end)
    else
      Map.update(house, key2, [], fn list -> list -- [value2] end)
    end
  end

  defp remove_properties({{key1, value1}, :next_to_the_right, {key2, value2}}, house) do
    Map.update(house, key1, [], fn list -> list -- [value1] end)
    |> Map.update(key2, [], fn list -> list -- [value2] end)
  end

  defp calc_state(statements, houses) do
    new_houses =
      Enum.reduce(statements, houses, fn statement, houses ->
        Enum.map(houses, fn house ->
          cond do
            invalid_house?(statement, house, houses) ->
              remove_properties(statement, house)

            valid_house?(statement, house, houses) ->
              set_properties(statement, house)

            true ->
              house
          end
        end)
      end)

    new_houses = remove_single_values(new_houses)

    if houses != new_houses do
      calc_state(statements, new_houses)
    else
      houses
    end
  end

  defp remove_single_values(houses) do
    Enum.reduce(houses, houses, fn current_house, acc ->
      Enum.reduce(Map.keys(current_house), acc, fn key, updated_houses ->
        case current_house[key] do
          [single_value] ->
            Enum.map(updated_houses, fn house ->
              if house.position == current_house.position do
                house
              else
                Map.update(house, key, [], fn existing_values ->
                  Enum.reject(existing_values, &(&1 == single_value))
                end)
              end
            end)

          _ ->
            updated_houses
        end
      end)
    end)
  end

  defp try_state(states, statements, property, val) do
    result =
      Enum.flat_map(states, fn houses ->
        possible_houses =
          Enum.filter(houses, fn house -> val in house[property] and house[property] != [val] end)

        states =
          Enum.map(possible_houses, fn possible_house ->
            possible_state =
              Enum.map(houses, fn house ->
                if house.position == possible_house.position do
                  Map.put(house, property, [val])
                else
                  Map.update(house, property, [], fn existing_values ->
                    Enum.reject(existing_values, &(&1 == val))
                  end)
                end
              end)

            calc_state(statements, possible_state)
          end)

        Enum.reject(states, fn state ->
          Enum.any?(state, fn house ->
            Enum.any?(house, fn {_key, value} -> value == [] end)
          end)
        end)
      end)

    if result == [] do
      states
    else
      result
    end
  end

  defp solve() do
    init_state = calc_state(@statements, @initial_state)

    result =
      Enum.reduce(@properties, [init_state], fn {key, values}, acc ->
        Enum.reduce(values, acc, fn value, acc ->
          try_state(acc, @statements, key, value)
        end)
      end)

    hd(result)
  end

  @doc """
  Determine who drinks the water
  """
  @spec drinks_water() :: atom
  def drinks_water() do
    [nationality] =
      Enum.find_value(solve(), fn house -> house[:drink] == [:water] && house[:nationality] end)

    nationality
  end

  @doc """
  Determine who owns the zebra
  """
  @spec owns_zebra() :: atom
  def owns_zebra() do
    [nationality] =
      Enum.find_value(solve(), fn house -> house[:pet] == [:zebra] && house[:nationality] end)

    nationality
  end
end
