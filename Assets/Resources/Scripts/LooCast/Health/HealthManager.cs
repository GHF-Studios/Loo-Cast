using System;
using UnityEngine;

namespace LooCast.Health
{
    using LooCast.System;
    using LooCast.System.Management;

    public class HealthManager : ModuleManager
    {
        #region Static Properties
        public static HealthManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[HealthManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<HealthManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static HealthManager instance;
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
            looCastNamespace = new Namespace("Health", rootNamespace);
            looCastType = new Type(typeof(HealthManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type allyHealthType = new Type(typeof(AllyHealth), looCastNamespace);
            Type allyStationHealthType = new Type(typeof(AllyStationHealth), looCastNamespace);
            Type damageInfoType = new Type(typeof(DamageInfo), looCastNamespace);
            Type enemyHealthType = new Type(typeof(EnemyHealth), looCastNamespace);
            Type enemyStationHealthType = new Type(typeof(EnemyStationHealth), looCastNamespace);
            Type iHealthType = new Type(typeof(IHealth), looCastNamespace);
            Type playerHealthType = new Type(typeof(PlayerHealth), looCastNamespace);

            typeManager.RegisterType(allyHealthType);
            typeManager.RegisterType(allyStationHealthType);
            typeManager.RegisterType(damageInfoType);
            typeManager.RegisterType(enemyHealthType);
            typeManager.RegisterType(enemyStationHealthType);
            typeManager.RegisterType(iHealthType);
            typeManager.RegisterType(playerHealthType);
            #endregion
        }
        #endregion
    }
}