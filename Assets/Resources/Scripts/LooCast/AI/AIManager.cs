using System;
using UnityEngine;

namespace LooCast.AI
{
    public class AIManager : ModuleManager
    {
        #region Static Properties
        public static AIManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[AIManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<AIManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static AIManager instance;
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
            looCastNamespace = new Namespace("AI", rootNamespace);
            looCastType = new Type(typeof(AIManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type allyAIType = new Type(typeof(AllyAI), looCastNamespace);
            Type enemyAIType = new Type(typeof(EnemyAI), looCastNamespace);

            typeManager.RegisterType(allyAIType);
            typeManager.RegisterType(enemyAIType);
            #endregion
        }
        #endregion
    }
}