using System;
using UnityEngine;

namespace LooCast.Station
{
    using LooCast.System;
    using LooCast.System.Management;

    public class StationManager : ModuleManager
    {
        #region Static Properties
        public static StationManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[StationManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<StationManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static StationManager instance;
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
            looCastNamespace = new Namespace("Station", rootNamespace);
            looCastType = new Type(typeof(StationManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type allyStationType = new Type(typeof(AllyStation), looCastNamespace);
            Type enemyStationType = new Type(typeof(EnemyStation), looCastNamespace);
            Type stationType = new Type(typeof(Station), looCastNamespace);

            typeManager.RegisterType(allyStationType);
            typeManager.RegisterType(enemyStationType);
            typeManager.RegisterType(stationType);
            #endregion
        }
        #endregion
    }
}