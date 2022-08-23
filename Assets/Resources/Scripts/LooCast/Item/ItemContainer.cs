using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    public sealed class ItemContainer
    {
        private Item[] itemSlots;

        public ItemContainer(int slotCount)
        {
            itemSlots = new Item[slotCount];
        }

        public ItemContainer(Item[] items)
        {
            itemSlots = items;
        }

        public bool SetItem(int slot, Item item)
        {
            if (IsValidSlot(slot))
            {
                itemSlots[slot] = item;
                return true;
            }
            return false;
        }

        public bool GetItem(int slot, out Item item)
        {
            if (IsValidSlot(slot))
            {
                item = itemSlots[slot];
                return true;
            }
            item = null;
            return false;
        }

        public Item[] GetItems()
        {
            return itemSlots;
        }

        public void Clear()
        {
            itemSlots = new Item[itemSlots.Length];
        }

        public bool Clear(int newSlotCount)
        {
            if (newSlotCount > 0)
            {
                itemSlots = new Item[newSlotCount];
                return true;
            }
            return false;
        }

        public bool IsValidSlot(int slot)
        {
            return slot < itemSlots.Length && slot > 0;
        }
    }
}