using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Hotbar
{
    using LooCast.Item;

    public class HotbarSlot : MonoBehaviour
    {
        public Item Item
        {
            get
            {
                return Item;
            }

            set
            {
                item = value;
                Refresh();
            }
        }
        
        [SerializeField] private Image image;
        [SerializeField] private Text quantityValue;

        private Item item;

        public void Refresh()
        {
            if (item == null)
            {
                image.enabled = false;
                image.sprite = null;
                quantityValue.enabled = false;
                quantityValue.text = "";
            }
            else if (item is CountableItem)
            {
                CountableItem countableItem = (CountableItem)item;
                image.enabled = true;
                image.sprite = countableItem.Sprite;
                quantityValue.enabled = true;
                quantityValue.text = $"{countableItem.Count}";
            }
            else if(item is AmountableItem)
            {
                AmountableItem amountableItem = (AmountableItem)item;
                image.enabled = true;
                image.sprite = amountableItem.Sprite;
                quantityValue.enabled = true;
                quantityValue.text = string.Format("{0:n0}", amountableItem.Amount) + "t";
            }
        }
    }
}
