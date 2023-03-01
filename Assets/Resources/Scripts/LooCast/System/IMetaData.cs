namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface IMetaData
    {
        #region Properties
        public string Name { get; }
        public string Description { get; }
        public string Author { get; }
        public string Version { get; }
        #endregion
    }
}
