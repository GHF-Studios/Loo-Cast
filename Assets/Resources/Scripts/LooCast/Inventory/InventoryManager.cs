using System;
using UnityEngine;

namespace LooCast.Inventory
{
    using LooCast.System;
    using LooCast.System.Management;

    public class InventoryManager : ModuleManager
    {
        #region Static Properties
        public static InventoryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[InventoryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<InventoryManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static InventoryManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Inventory", rootNamespace);
            looCastType = new Type(typeof(InventoryManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type allyStationInventoryType = new Type(typeof(AllyStationInventory), looCastNamespace);
            Type enemyStationInventoryType = new Type(typeof(EnemyStationInventory), looCastNamespace);
            Type playerInventoryType = new Type(typeof(PlayerInventory), looCastNamespace);

            typeManager.RegisterType(allyStationInventoryType);
            typeManager.RegisterType(enemyStationInventoryType);
            typeManager.RegisterType(playerInventoryType);
            #endregion
        }
        #endregion
    }
}