using System;
using UnityEngine;

namespace LooCast.UI.Inventory
{
    public class InventorySlotCursor : MonoBehaviour
    {
        #region Properties
        public InventorySlot CurrentInventorySlot
        {
            get
            {
                return currentInventorySlot;
            }

            set
            {
                if (value == null)
                {
                    throw new NullReferenceException("Current Inventory Slot cannot be null!"); 
                }
                currentInventorySlot = value;
                rectTransform.SetParent(currentInventorySlot.RectTransform);
                rectTransform.SetAsFirstSibling();
                rectTransform.anchoredPosition = Vector2.zero;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private InventorySlot currentInventorySlot;
        [SerializeField] private RectTransform rectTransform;
        #endregion
    }
}