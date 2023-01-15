using System;
using System.Collections.Generic;

namespace LooCast.Core
{
    public class ModuleManager
    {
        #region Static properties
        public static ModuleManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ModuleManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ModuleManager instance;
        #endregion

        #region Fields
        private Dictionary<string, Module> registeredModules;
        #endregion

        #region Methods
        public void RegisterModule(Module module)
        {
            if (registeredModules.ContainsKey(module.Name))
            {
                throw new Exception("[ModuleManager] Module with ID" + module.Name + " has already been registered.");
            }
            registeredModules.Add(module.Name, module);
        }

        public void UnregisterModule(Module module)
        {
            if (!registeredModules.ContainsKey(module.Name))
            {
                throw new Exception("[ModuleManager] Module with ID" + module.Name + " has not been registered.");
            }
            registeredModules.Remove(module.Name);
        }
        #endregion
    }
