using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using LooCast.Core;
    using LooCast.Inventory;

    [RequireComponent(typeof(SpriteRenderer))]
    public abstract class ItemObject : Component
    {
        public UnityEvent OnItemChanged
        {
            get
            {
                return onItemChanged;
            }
        }
        public virtual Item Item
        {
            get
            {
                return item;
            }

            set
            {
                item = value;
                OnItemChanged.Invoke();
                SpriteRenderer.sprite = item.Sprite;
            }
        }
        public SpriteRenderer SpriteRenderer { get; protected set; }

        private UnityEvent onItemChanged;
        private Item item;

        protected void Initialize(Item item)
        {
            onItemChanged = new UnityEvent();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Item = item;
        }

        private void OnTriggerStay2D(Collider2D collider)
        {
            if (collider.CompareTag("Player"))
            {
                PlayerInventory playerInventory = collider.GetComponent<PlayerInventory>();
                ItemContainer playerItemContainer = playerInventory.RuntimeData.Hotbar;

                playerItemContainer.TryAddItem(Item, out Item remainingItem);
                if (remainingItem != null)
                {
                    Item = remainingItem;
                }
            }
        }
    }
}