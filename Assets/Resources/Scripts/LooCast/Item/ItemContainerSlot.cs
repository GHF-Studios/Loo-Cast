using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    public class ItemContainerSlot
    {
        public ItemContainer ItemContainer { get; private set; }
        public Item ItemContent
        {
            get
            {
                return itemContent;
            }

            set
            {
                itemContent = value;
                if (itemContent != null)
                {
                    if (itemContent.ItemContainmentState != Item.ContainmentState.Contained)
                    {
                        itemContent.ContainItem(ItemContainer);
                    }
                }
                OnItemContentChanged.Invoke();
            }
        }
        public UnityEvent OnItemContentChanged { get; private set; }
        private Item itemContent;

        public ItemContainerSlot(ItemContainer itemContainer, Item itemContent = null)
        {
            ItemContainer = itemContainer;
            OnItemContentChanged = new UnityEvent();
            this.itemContent = itemContent;
        }
    }
}