using System;
using UnityEngine;

namespace LooCast.Generator
{
    using LooCast.System;
    using LooCast.System.Management;

    public class GeneratorManager : ModuleManager
    {
        #region Static Properties
        public static GeneratorManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[GeneratorManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<GeneratorManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static GeneratorManager instance;
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
            looCastNamespace = new Namespace("Generator", rootNamespace);
            looCastType = new Type(typeof(GeneratorManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type allyStationGeneratorType = new Type(typeof(AllyStationGenerator), looCastNamespace);
            Type asteroidGeneratorType = new Type(typeof(AsteroidGenerator), looCastNamespace);
            Type enemyStationGeneratorType = new Type(typeof(EnemyStationGenerator), looCastNamespace);
            Type generatorType = new Type(typeof(Generator), looCastNamespace);
            Type generatorsType = new Type(typeof(Generators), looCastNamespace);

            typeManager.RegisterType(allyStationGeneratorType);
            typeManager.RegisterType(asteroidGeneratorType);
            typeManager.RegisterType(enemyStationGeneratorType);
            typeManager.RegisterType(generatorType);
            typeManager.RegisterType(generatorsType);
            #endregion
        }
        #endregion
    }
}