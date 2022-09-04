using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using LooCast.Util;

    public class WeaponItemContainer : ItemContainer
    {
        public WeaponItemContainer(int slotCount, GameObject originObject = null) : base(slotCount, originObject)
        {
            
        }

        public WeaponItemContainer(WeaponItem[] weaponItems, GameObject originObject = null) : base(weaponItems, originObject)
        {
            
        }

        #region New (Dirty Method-Parameter Polymorphism) Methods
        //This is really dirty and should be changed, but it works
        //Goal: Remove ItemContainer's inherited class's Implementation's need for this Method Hiding, having a single Method to Add Items in the end,
        //which still has a restriction implied upon it, based upon the ItemContainer's Inherited Class in which the Method is contained
        //Explanation: This Method hides ItemContainer's AddItem(Item item, out Item remainingItem)-Method, so there is zero risk
        //of someone accidentally using this Method on a WeaponItemContainer (which is not intended)
        //A solution would be to add a generic type argument to ItemContainer, but that's dirty too and also has some other use-case specific problems, trust me bro
        //Another solution would be to add a generic type argument to the Method itself, but the Type Restrictions are implicitly inherited from the
        //base class's implementation and can thus not be explicitely defined in the inherited class's method override.
        //In Layman's Terms: You can't do ItemContainer.AddItem<T> where T : Item,
        //and then do WeaponItemContainer.AddItem<T> where T : WeaponItem, even though WeaponItem is inherited from Item, meaning:
        //No polymorphism available for a Method's Generic Type Argument Restriction, only for a Class's Generic Type Argument Restriction


        //Dirty Hide-Method, DON'T USE!
        private new void AddItem(Item item, out Item remainingItem)
        {
            throw new InvalidOperationException("Do NOT call this Method!");
        }
        public void AddItem(WeaponItem weaponItem, out WeaponItem remainingWeaponItem)
        {
            base.AddItem(weaponItem, out Item item);
            remainingWeaponItem = (WeaponItem)item;
        }
        
        //Dirty Hide-Method, DON'T USE!
        private new void SetItem(int slotID, Item item)
        {
            throw new InvalidOperationException("Do NOT call this Method!");
        }
        public void SetItem(int slotID, WeaponItem weaponItem)
        {
            base.SetItem(slotID, weaponItem);
        }

        //Dirty Hide-Method, DON'T USE!
        private new bool Contains(Item item)
        {
            throw new InvalidOperationException("Do NOT call this Method!");
        }

        public bool Contains(WeaponItem weaponItem)
        {
            return base.Contains(weaponItem);
        }
        #endregion    

        #region New Methods
        public new WeaponItem GetItem(int slotID)
        {
            return (WeaponItem)base.GetItem(slotID);
        }

        public new WeaponItem[] GetItems()
        {
            return itemSlots.GetItems().Cast<WeaponItem>();
        }
        #endregion
    }
}
