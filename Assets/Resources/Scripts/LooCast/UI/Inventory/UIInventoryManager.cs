using System;
using UnityEngine;

namespace LooCast.UI.Inventory
{
    public class UIInventoryManager : SubModuleManager
    {
        #region Static Properties
        public static UIInventoryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIInventoryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIInventoryManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIInventoryManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Inventory", rootNamespace);
            looCastType = new Type(typeof(UIInventoryManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type inventoryHotbarType = new Type(typeof(InventoryHotbar), looCastNamespace);
            Type inventoryItemType = new Type(typeof(InventoryItem), looCastNamespace);
            Type inventorySlotType = new Type(typeof(InventorySlot), looCastNamespace);
            Type inventorySlotCursorType = new Type(typeof(InventorySlotCursor), looCastNamespace);

            typeManager.RegisterType(inventoryHotbarType);
            typeManager.RegisterType(inventoryItemType);
            typeManager.RegisterType(inventorySlotType);
            typeManager.RegisterType(inventorySlotCursorType);
            #endregion
        }
        #endregion
    }
}