using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Item
{
    public sealed class ItemContainer
    {
        private Item[] itemSlots;
        public UnityEvent OnContentChanged
        {
            get
            {
                return onContentChanged;
            }
        }
        private UnityEvent onContentChanged;

        public ItemContainer(int slotCount)
        {
            if (slotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }
            itemSlots = new Item[slotCount];
            onContentChanged = new UnityEvent();
        }

        public ItemContainer(Item[] items)
        {
            if (items == null)
            {
                throw new ArgumentNullException("Items cannot be null!");
            }
            if (items.Length == 0)
            {
                throw new ArgumentOutOfRangeException("Items must have atleast one entry!");
            }
            itemSlots = items;
            onContentChanged = new UnityEvent();
        }

        public void AddItem(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            else if (item is CountableItem)
            {
                AddItem((CountableItem)item);
                onContentChanged.Invoke();
            }
            else if (item is AmountableItem)
            {
                AddItem((AmountableItem)item);
                onContentChanged.Invoke();
            }
            else if (item is UniqueItem)
            {
                AddItem((UniqueItem)item);
                onContentChanged.Invoke();
            }
            else
            {
                throw new NotSupportedException("Unsupported Item Type!");
            }
        }

        private void AddItem(CountableItem countableItem)
        {
            if (CanFit(countableItem))
            {
                int countToAdd = countableItem.Count;
                for (int i = 0; i < itemSlots.Length; i++)
                {
                    if (itemSlots[i] == null)
                    {
                        itemSlots[i] = countableItem;
                    }
                    else if (itemSlots[i].Equals(countableItem))
                    {
                        CountableItem itemSlot = (CountableItem)itemSlots[i];
                        if (!itemSlot.IsFull())
                        {
                            int freeCount = itemSlot.GetFreeCount();
                            if (countToAdd > freeCount)
                            {
                                itemSlot.Count = itemSlot.MaxCount;
                                countToAdd -= freeCount;
                            }
                            else if (countToAdd < freeCount)
                            {
                                itemSlot.Count += countToAdd;
                                return;
                            }
                            else
                            {
                                itemSlot.Count = itemSlot.MaxCount;
                                return;
                            }
                        }
                    }
                }
            }
        }

        private void AddItem(AmountableItem amountableItem)
        {
            if (CanFit(amountableItem))
            {
                float amountToAdd = amountableItem.Amount;
                for (int i = 0; i < itemSlots.Length; i++)
                {
                    if (itemSlots[i] == null)
                    {
                        itemSlots[i] = amountableItem;
                    }
                    else if (itemSlots[i].Equals(amountableItem))
                    {
                        AmountableItem itemSlot = (AmountableItem)itemSlots[i];
                        if (!itemSlot.IsFull())
                        {
                            float freeAmount = itemSlot.GetFreeAmount();
                            if (amountToAdd > freeAmount)
                            {
                                itemSlot.Amount = itemSlot.MaxAmount;
                                amountToAdd -= freeAmount;
                            }
                            else if (amountToAdd < freeAmount)
                            {
                                itemSlot.Amount += amountToAdd;
                                return;
                            }
                            else
                            {
                                itemSlot.Amount = itemSlot.MaxAmount;
                                return;
                            }
                        }
                    }
                }
            }
        }

        private void AddItem(UniqueItem uniqueItem)
        {
            if (CanFit(uniqueItem))
            {
                for (int i = 0; i < itemSlots.Length; i++)
                {
                    if (itemSlots[i] == null)
                    {
                        itemSlots[i] = uniqueItem;
                        return;
                    }
                }
            }
        }

        public void SetItem(int slot, Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            if (!IsValidSlot(slot))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Length - 1}!");
            }
            itemSlots[slot] = item;
            onContentChanged.Invoke();
        }

        public Item GetItem(int slot)
        {
            if (!IsValidSlot(slot))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Length - 1}!");
            }
            return itemSlots[slot];
        }

        public Item[] GetItems()
        {
            return itemSlots;
        }

        public bool Contains(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            foreach (Item itemSlot in itemSlots)
            {
                if (itemSlot.Equals(item))
                {
                    return true;
                }
            }
            return false;
        }

        public bool CanFit(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            else if (item is CountableItem)
            {
                return CanFit((CountableItem)item);
            }
            else if (item is AmountableItem)
            {
                return CanFit((AmountableItem)item);
            }
            else if (item is UniqueItem)
            {
                return CanFit((UniqueItem)item);
            }
            else
            {
                throw new NotSupportedException("Unsupported Item Type!");
            }
        }

        private bool CanFit(CountableItem countableItem)
        {
            List<CountableItem> partiallyVacantSlots = new List<CountableItem>();
            for (int i = 0; i < itemSlots.Length; i++)
            {
                if (itemSlots[i] == null)
                {
                    return true;
                }
                else if (itemSlots[i].Equals(countableItem))
                {
                    CountableItem potentiallyVacantSlot = (CountableItem)itemSlots[i];
                    if (!potentiallyVacantSlot.IsFull())
                    {
                        partiallyVacantSlots.Add(potentiallyVacantSlot);
                    }
                }
            }

            if (partiallyVacantSlots.Count == 0)
            {
                return false;
            }

            int countToAdd = countableItem.Count;
            foreach (CountableItem partiallyVacantSlot in partiallyVacantSlots)
            {
                countToAdd -= partiallyVacantSlot.GetFreeCount();
                if (countToAdd <= 0)
                {
                    return true;
                }
            }
            return false;
        }

        private bool CanFit(AmountableItem amountableItem)
        {
            List<AmountableItem> partiallyVacantSlots = new List<AmountableItem>();
            for (int i = 0; i < itemSlots.Length; i++)
            {
                if (itemSlots[i] == null)
                {
                    return true;
                }
                else if (itemSlots[i].Equals(amountableItem))
                {
                    AmountableItem potentiallyVacantSlot = (AmountableItem)itemSlots[i];
                    if (!potentiallyVacantSlot.IsFull())
                    {
                        partiallyVacantSlots.Add(potentiallyVacantSlot);
                    }
                }
            }

            if (partiallyVacantSlots.Count == 0)
            {
                return false;
            }

            float amountToAdd = amountableItem.Amount;
            foreach (AmountableItem partiallyVacantSlot in partiallyVacantSlots)
            {
                amountToAdd -= partiallyVacantSlot.GetFreeAmount();
                if (amountToAdd <= 0.0f)
                {
                    return true;
                }
            }
            return false;
        }

        private bool CanFit(UniqueItem uniqueItem)
        {
            for (int i = 0; i < itemSlots.Length; i++)
                {
                    if (itemSlots[i] == null)
                    {
                        return true;
                    }
                }
                return false;
        }

        public void Clear()
        {
            itemSlots = new Item[itemSlots.Length];
            onContentChanged.Invoke();
        }

        public void Clear(int newSlotCount)
        {
            if (newSlotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }
            itemSlots = new Item[newSlotCount];
            onContentChanged.Invoke();
        }

        public bool IsValidSlot(int slot)
        {
            return slot < itemSlots.Length && slot >= 0;
        }
    }
}