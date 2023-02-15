using System;
using UnityEngine;

namespace LooCast.UI.Panel
{
    public class UIPanelManager : SubModuleManager
    {
        #region Static Properties
        public static UIPanelManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIPanelManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIPanelManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIPanelManager instance;
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
            looCastNamespace = new Namespace("Panel", rootNamespace);
            looCastType = new Type(typeof(UIPanelManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            
            Type asteroidInfoPanelType = new Type(typeof(AsteroidInfoPanel), looCastNamespace);
            Type asteroidResourceDepositsPanelType = new Type(typeof(AsteroidResourceDepositsPanel), looCastNamespace);
            Type panelType = new Type(typeof(Panel), looCastNamespace);
            Type stationBlackmarketPanelType = new Type(typeof(StationBlackmarketPanel), looCastNamespace);
            Type stationHUBPanelType = new Type(typeof(StationHUBPanel), looCastNamespace);
            Type stationManufacturingPanelType = new Type(typeof(StationManufacturingPanel), looCastNamespace);
            Type stationMarketPanelType = new Type(typeof(StationMarketPanel), looCastNamespace);
            Type stationMissionPanelType = new Type(typeof(StationMissionPanel), looCastNamespace);
            Type stationUpgradesPaneltype = new Type(typeof(StationUpgradesPanel), looCastNamespace);

            typeManager.RegisterType(asteroidInfoPanelType);
            typeManager.RegisterType(asteroidResourceDepositsPanelType);
            typeManager.RegisterType(panelType);
            typeManager.RegisterType(stationBlackmarketPanelType);
            typeManager.RegisterType(stationHUBPanelType);
            typeManager.RegisterType(stationManufacturingPanelType);
            typeManager.RegisterType(stationMarketPanelType);
            typeManager.RegisterType(stationMissionPanelType);
            typeManager.RegisterType(stationUpgradesPaneltype);
            #endregion
        }
        #endregion
    }
}