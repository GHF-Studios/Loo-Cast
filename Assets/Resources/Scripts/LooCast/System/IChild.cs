namespace LooCast.System
{
    public interface IChild<TParent>
    {
        #region Properties
        TParent Parent { get; }
        #endregion
    }
}
