using System;
using UnityEngine;

namespace LooCast.Arc
{
    using LooCast.System;
    using LooCast.System.Management;

    public class ArcManager : ModuleManager
    {
        #region Static Properties
        public static ArcManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[ArcManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<ArcManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ArcManager instance;
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
            looCastNamespace = new Namespace("Arc", rootNamespace);
            looCastType = new Type(typeof(ArcManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type arcType = new Type(typeof(Arc), looCastNamespace);
            Type arcSegmentType = new Type(typeof(ArcSegment), looCastNamespace);

            typeManager.RegisterType(arcType);
            typeManager.RegisterType(arcSegmentType);
            #endregion
        }
        #endregion
    }
}