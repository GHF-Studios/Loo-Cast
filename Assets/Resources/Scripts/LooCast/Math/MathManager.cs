using LooCast.Math.Map;
using System;
using UnityEngine;

namespace LooCast.Math
{
    public class MathManager : ModuleManager
    {
        #region Static Properties
        public static MathManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[MathManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<MathManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MathManager instance;
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
            looCastNamespace = new Namespace("Math", rootNamespace);
            Namespace mapNamespace = new Namespace("Map", looCastNamespace);
            looCastType = new Type(typeof(MathManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            namespaceManager.RegisterNamespace(mapNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type floatMap2DType = new Type(typeof(FloatMap2D), mapNamespace);

            typeManager.RegisterType(floatMap2DType);
            #endregion
        }
        #endregion
    }
}