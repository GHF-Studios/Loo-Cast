using System;
using UnityEngine;

namespace LooCast.Core.Type
{
    using Identifier;

    public class TypeManager : SubModuleManager
    {
        #region Static Properties
        public static TypeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[TypeManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<TypeManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static TypeManager instance;
        #endregion

        #region Properties
        public override Manager[] SubManagers => subManagers;
        #endregion

        #region Fields
        private Manager[] subManagers;
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            subManagers = new Manager[]
            {

            };
        }

        public override void Initialize()
        {

        }

        public override void PostInitialize()
        {

        }
        
        public void RegisterType(Type type)
        {
            
        }

        public Type GetType(TypeIdentifier typeIdentifier)
        {

        }
        #endregion
    }
}