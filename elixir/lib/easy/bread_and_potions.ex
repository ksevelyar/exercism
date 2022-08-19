defmodule RPG do
  defmodule Character do
    defstruct health: 100, mana: 0
  end

  defmodule LoafOfBread do
    defstruct []
  end

  defmodule ManaPotion do
    defstruct strength: 10
  end

  defmodule Poison do
    defstruct []
  end

  defmodule EmptyBottle do
    defstruct []
  end

  defprotocol Edible do
    def eat(item, char)
  end

  defimpl Edible, for: LoafOfBread do
    def eat(_, %{health: health} = char) do
      {nil, %{char | health: health + 5}}
    end
  end

  defimpl Edible, for: ManaPotion do
    def eat(%{strength: potion_mana}, %{mana: char_mana} = char) do
      {%RPG.EmptyBottle{}, %{char | mana: potion_mana + char_mana}}
    end
  end

  defimpl Edible, for: Poison do
    def eat(_, char) do
      {%RPG.EmptyBottle{}, %{char | health: 0}}
    end
  end
end
