using System;
using UnityEngine;

namespace LooCast.Enemy
{
    using LooCast.System;
    using LooCast.System.Management;

    public class EnemyManager : ModuleManager
    {
        #region Static Properties
        public static EnemyManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[EnemyManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<EnemyManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static EnemyManager instance;
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
            looCastNamespace = new Namespace("Enemy", rootNamespace);
            looCastType = new Type(typeof(EnemyManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type enemyType = new Type(typeof(Enemy), looCastNamespace);
            Type smolEnemyType = new Type(typeof(SmolEnemy), looCastNamespace);

            typeManager.RegisterType(enemyType);
            typeManager.RegisterType(smolEnemyType);
            #endregion
        }
        #endregion
    }
}