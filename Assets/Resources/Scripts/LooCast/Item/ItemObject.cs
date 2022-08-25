using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using LooCast.Core;
    using LooCast.Inventory;

    [RequireComponent(typeof(SpriteRenderer))]
    public abstract class ItemObject : ExtendedMonoBehaviour
    {
        public UnityEvent OnItemChanged
        {
            get
            {
                return onItemChanged;
            }
        }
        public Item Item
        {
            get
            {
                return item;
            }

            set
            {
                item = value;
                onItemChanged.Invoke();
            }
        }
        public SpriteRenderer SpriteRenderer { get; protected set; }

        private UnityEvent onItemChanged;
        private Item item;

        protected void Initialize(Item item)
        {
            onItemChanged = new UnityEvent();

            Item = item;
            SpriteRenderer = GetComponent<SpriteRenderer>();
            SpriteRenderer.sprite = item.Sprite;
        }

        private void OnTriggerEnter2D(Collider2D collider)
        {
            if (collider.CompareTag("Player"))
            {
                PlayerInventory playerInventory = collider.GetComponent<PlayerInventory>();
                ItemContainer playerItemContainer = playerInventory.RuntimeData.ItemContainer;

                playerItemContainer.AddItem(Item, out Item remainingItem);
                if (remainingItem != null)
                {
                    Item = remainingItem;
                }
                else
                {
                    Destroy(gameObject);
                }
            }
        }
    }
}