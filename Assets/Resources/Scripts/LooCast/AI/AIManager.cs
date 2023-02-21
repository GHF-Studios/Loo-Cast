using System;
using UnityEngine;

namespace LooCast.AI
{
    using LooCast.System;
    using LooCast.System.Management;

    public class AIManager : ModuleManager
    {
        #region Static Properties
        public static AIManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[AIManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("AI", rootNamespace);
            looCastType = new Type(typeof(AIManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type allyAIType = new Type(typeof(AllyAI), looCastNamespace);
            Type enemyAIType = new Type(typeof(EnemyAI), looCastNamespace);

            typeManager.RegisterType(allyAIType);
            typeManager.RegisterType(enemyAIType);
            #endregion
        }
        #endregion
    }
}