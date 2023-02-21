using System;
using UnityEngine;

namespace LooCast.Event
{
    using LooCast.System;
    using LooCast.System.Management;
    
    public class EventManager : ModuleManager
    {
        #region Static Properties
        public static EventManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[EventManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<EventManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static EventManager instance;
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
            looCastNamespace = new Namespace("Event", rootNamespace);
            looCastType = new Type(typeof(EventManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type eventType = new Type(typeof(Event), looCastNamespace);
            Type eventListenerType = new Type(typeof(EventListener), looCastNamespace);

            typeManager.RegisterType(eventType);
            typeManager.RegisterType(eventListenerType);
            #endregion
        }
        #endregion
    }
}