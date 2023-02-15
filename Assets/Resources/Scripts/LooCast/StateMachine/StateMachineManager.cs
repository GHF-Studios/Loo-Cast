using System;
using UnityEngine;

namespace LooCast.StateMachine
{
    public class StateMachineManager : ModuleManager
    {
        #region Static Properties
        public static StateMachineManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[StateMachineManager]");
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
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("StateMachine", rootNamespace);
            looCastType = new Type(typeof(StateMachineManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            
            Type finiteStateMachineType = new Type(typeof(FiniteStateMachine<State<object>>), looCastNamespace);
            Type stateType = new Type(typeof(State<object>), looCastNamespace);

            typeManager.RegisterType(finiteStateMachineType);
            typeManager.RegisterType(stateType);
            #endregion
        }
        #endregion
    }
}