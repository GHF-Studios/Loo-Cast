using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    public class ItemContainerSlot
    {
        public Item ItemContent
        {
            get
            {
                return itemContent;
            }

            set
            {
                OnItemContentChanged.Invoke();
                itemContent = value;
            }
        }
        public UnityEvent OnItemContentChanged { get; private set; }
        private Item itemContent;

        public ItemContainerSlot(Item itemContent = null)
        {
            OnItemContentChanged = new UnityEvent();
            this.itemContent = itemContent;
        }
    }
}