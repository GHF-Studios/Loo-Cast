using System;
using UnityEngine;

namespace LooCast.Item
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class ItemManager : ModuleManager
    {
        #region Static Properties
        public static ItemManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[ItemManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<ItemManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ItemManager instance;
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Item", rootNamespace);
            looCastType = new Type(typeof(ItemManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type itemType = new Type(typeof(Item), looCastNamespace);
            Type itemObjectType = new Type(typeof(ItemObject), looCastNamespace);
            Type itemContainerType = new Type(typeof(ItemContainer), looCastNamespace);
            Type itemContainerSlotType = new Type(typeof(ItemContainerSlot), looCastNamespace);
            Type amountableItemType = new Type(typeof(AmountableItem), looCastNamespace);
            Type amountableItemObject = new Type(typeof(AmountableItemObject), looCastNamespace);
            Type countableItemType = new Type(typeof(CountableItem), looCastNamespace);
            Type countableItemObjectType = new Type(typeof(CountableItemObject), looCastNamespace);
            Type uniqueItemType = new Type(typeof(UniqueItem), looCastNamespace);
            Type uniqueItemObjectType = new Type(typeof(UniqueItemObject), looCastNamespace);
            Type upgradableItemType = new Type(typeof(UpgradableItem), looCastNamespace);
            Type upgradableItemObjectType = new Type(typeof(UpgradableItemObject), looCastNamespace);
            Type upgradeSetType = new Type(typeof(UpgradeSet), looCastNamespace);
            Type iItemUpgraderType = new Type(typeof(IItemUpgrader), looCastNamespace);
            Type resourceItemType = new Type(typeof(ResourceItem), looCastNamespace);
            Type resourceItemObjectType = new Type(typeof(ResourceItemObject), looCastNamespace);
            Type weaponItemType = new Type(typeof(WeaponItem), looCastNamespace);
            Type weaponItemObjectType = new Type(typeof(WeaponItemObject), looCastNamespace);
            Type weaponItemContainerType = new Type(typeof(WeaponItemContainer), looCastNamespace);
            Type chargedPlasmaLauncherWeaponItemType = new Type(typeof(ChargedPlasmaLauncherWeaponItem), looCastNamespace);
            Type chargedPlasmaLauncherWeaponItemObjectType = new Type(typeof(ChargedPlasmaLauncherWeaponItemObject), looCastNamespace);
            Type freezeRayWeaponItemType = new Type(typeof(FreezeRayWeaponItem), looCastNamespace);
            Type freezeRayWeaponItemObjectType = new Type(typeof(FreezeRayWeaponItemObject), looCastNamespace);
            Type laserEmitterWeaponItemType = new Type(typeof(LaserEmitterWeaponItem), looCastNamespace);
            Type laserEmitterWeaponItemObjectType = new Type(typeof(LaserEmitterWeaponItemObject), looCastNamespace);
            Type multiplexerWeaponItemType = new Type(typeof(MultiplexerWeaponItem), looCastNamespace);
            Type multiplexerWeaponItemObjectType = new Type(typeof(MultiplexerWeaponItemObject), looCastNamespace);

            typeManager.RegisterType(itemType);
            typeManager.RegisterType(itemObjectType);
            typeManager.RegisterType(itemContainerType);
            typeManager.RegisterType(itemContainerSlotType);
            typeManager.RegisterType(amountableItemType);
            typeManager.RegisterType(amountableItemObject);
            typeManager.RegisterType(countableItemType);
            typeManager.RegisterType(countableItemObjectType);
            typeManager.RegisterType(uniqueItemType);
            typeManager.RegisterType(uniqueItemObjectType);
            typeManager.RegisterType(upgradableItemType);
            typeManager.RegisterType(upgradableItemObjectType);
            typeManager.RegisterType(upgradeSetType);
            typeManager.RegisterType(iItemUpgraderType);
            typeManager.RegisterType(resourceItemType);
            typeManager.RegisterType(resourceItemObjectType);
            typeManager.RegisterType(weaponItemType);
            typeManager.RegisterType(weaponItemObjectType);
            typeManager.RegisterType(weaponItemContainerType);
            typeManager.RegisterType(chargedPlasmaLauncherWeaponItemType);
            typeManager.RegisterType(chargedPlasmaLauncherWeaponItemObjectType);
            typeManager.RegisterType(freezeRayWeaponItemType);
            typeManager.RegisterType(freezeRayWeaponItemObjectType);
            typeManager.RegisterType(laserEmitterWeaponItemType);
            typeManager.RegisterType(laserEmitterWeaponItemObjectType);
            typeManager.RegisterType(multiplexerWeaponItemType);
            typeManager.RegisterType(multiplexerWeaponItemObjectType);
            #endregion
        }
        #endregion
    }
}