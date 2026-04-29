using System;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class TestManager : ModuleManager
    {
        #region Static Properties
        public static TestManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[TestManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<TestManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static TestManager instance;
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
            looCastNamespace = new Namespace("Test", rootNamespace);
            looCastType = new Type(typeof(TestManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type mapDisplayType = new Type(typeof(MapDisplay), looCastNamespace);
            Type perlinMapGeneratorType = new Type(typeof(PerlinMapGenerator), looCastNamespace);
            Type perlinMapGeneratorGPUType = new Type(typeof(PerlinMapGeneratorGPU), looCastNamespace);
            Type voronoiMapGeneratorType = new Type(typeof(VoronoiMapGenerator), looCastNamespace);
            Type voronoiMapGeneratorGPUType = new Type(typeof(VoronoiMapGeneratorGPU), looCastNamespace);

            typeManager.RegisterType(mapDisplayType);
            typeManager.RegisterType(perlinMapGeneratorType);
            typeManager.RegisterType(perlinMapGeneratorGPUType);
            typeManager.RegisterType(voronoiMapGeneratorType);
            typeManager.RegisterType(voronoiMapGeneratorGPUType);
            #endregion
        }
        #endregion
    }
}