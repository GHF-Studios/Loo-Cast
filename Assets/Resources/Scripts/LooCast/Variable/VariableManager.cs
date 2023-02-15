using System;
using UnityEngine;

namespace LooCast.Variable
{
    public class VariableManager : ModuleManager
    {
        #region Static Properties
        public static VariableManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[VariableManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<VariableManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static VariableManager instance;
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
            looCastNamespace = new Namespace("Variable", rootNamespace);
            looCastType = new Type(typeof(VariableManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type variableType = new Type(typeof(Variable<object>), looCastNamespace);
            Type computedVariableType = new Type(typeof(ComputedVariable<object>), looCastNamespace);
            Type computedVariableUtilType = new Type(typeof(ComputedVariableUtil), looCastNamespace);
            Type increaseType = new Type(typeof(Increase), looCastNamespace);
            Type multiplierType = new Type(typeof(Multiplier), looCastNamespace);
            Type temporaryIncreaseType = new Type(typeof(TemporaryIncrease), looCastNamespace);
            Type temporaryMultiplierType = new Type(typeof(TemporaryMultiplier), looCastNamespace);
            Type boolVariableType = new Type(typeof(BoolVariable), looCastNamespace);
            Type floatVariableType = new Type(typeof(FloatVariable), looCastNamespace);
            Type floatComputedVariableType = new Type(typeof(FloatComputedVariable), looCastNamespace);
            Type intVariableType = new Type(typeof(IntVariable), looCastNamespace);
            Type intComputedVariableType = new Type(typeof(IntComputedVariable), looCastNamespace);
            Type stringVariableType = new Type(typeof(StringVariable), looCastNamespace);

            typeManager.RegisterType(variableType);
            typeManager.RegisterType(computedVariableType);
            typeManager.RegisterType(computedVariableUtilType);
            typeManager.RegisterType(increaseType);
            typeManager.RegisterType(multiplierType);
            typeManager.RegisterType(temporaryIncreaseType);
            typeManager.RegisterType(temporaryMultiplierType);
            typeManager.RegisterType(boolVariableType);
            typeManager.RegisterType(floatVariableType);
            typeManager.RegisterType(floatComputedVariableType);
            typeManager.RegisterType(intVariableType);
            typeManager.RegisterType(intComputedVariableType);
            typeManager.RegisterType(stringVariableType);
            #endregion
        }
        #endregion
    }
}