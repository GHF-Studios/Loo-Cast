using System;

namespace LooCast.Core
{
    public abstract class Module
    {
        #region Properties
        public string Name { get; set; }
        #endregion

        #region Methods
        public virtual void Initialize()
        {
            
        }
        
        public virtual void OnPreInitialize()
        {
            
        }
        
        public virtual void OnPostInitialize()
        {
            
        }
        #endregion
    }
}
