using CSSystem = System;
using UnityEngine;

namespace LooCast.StateMachine
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class StateMachineManager : ModuleManager
    {
        #region Static Properties
        public static StateMachineManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[StateMachineManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<StateMachineManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static StateMachineManager instance;
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
            looCastNamespace = new Namespace("StateMachine", rootNamespace);
            looCastType = new Type(typeof(StateMachineManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            
            Type finiteStateMachineType = new Type(typeof(FiniteStateMachine<CSSystem.Enum>), looCastNamespace);
            Type stateType = new Type(typeof(State<CSSystem.Enum>), looCastNamespace);

            typeManager.RegisterType(finiteStateMachineType);
            typeManager.RegisterType(stateType);
            #endregion
        }
        #endregion
    }
}