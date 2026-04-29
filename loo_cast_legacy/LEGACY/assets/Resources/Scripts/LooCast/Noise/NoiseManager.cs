using System;
using UnityEngine;

namespace LooCast.Noise
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class NoiseManager : ModuleManager
    {
        #region Static Properties
        public static NoiseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[NoiseManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<NoiseManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static NoiseManager instance;
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
            looCastNamespace = new Namespace("Noise", rootNamespace);
            looCastType = new Type(typeof(NoiseManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type fastNoiseLiteType = new Type(typeof(FastNoiseLite), looCastNamespace);
            Type perlinNoiseType = new Type(typeof(PerlinNoise), looCastNamespace);
            Type voronoiNoiseType = new Type(typeof(VoronoiNoise), looCastNamespace);

            typeManager.RegisterType(fastNoiseLiteType);
            typeManager.RegisterType(perlinNoiseType);
            typeManager.RegisterType(voronoiNoiseType);
            #endregion
        }
        #endregion
    }
}