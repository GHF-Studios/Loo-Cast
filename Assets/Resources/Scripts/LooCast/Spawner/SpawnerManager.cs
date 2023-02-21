using System;
using UnityEngine;

namespace LooCast.Spawner
{
    using LooCast.System;
    using LooCast.System.Management;

    public class SpawnerManager : ModuleManager
    {
        #region Static Properties
        public static SpawnerManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[SpawnerManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<SpawnerManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static SpawnerManager instance;
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
            looCastNamespace = new Namespace("Spawner", rootNamespace);
            looCastType = new Type(typeof(SpawnerManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type allySpawnerType = new Type(typeof(AllySpawner), looCastNamespace);
            Type enemySpawnerType = new Type(typeof(EnemySpawner), looCastNamespace);
            Type spawnerType = new Type(typeof(Spawner), looCastNamespace);

            typeManager.RegisterType(allySpawnerType);
            typeManager.RegisterType(enemySpawnerType);
            typeManager.RegisterType(spawnerType);
            #endregion
        }
        #endregion
    }
}