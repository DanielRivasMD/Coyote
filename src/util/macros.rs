////////////////////////////////////////////////////////////////////////////////////////////////////

/// Talus
/// In Greek mythology, Talos — also spelled Talus (/ˈteɪlɒs/; Greek: Τάλως,
/// Tálōs) or Talon (/ˈteɪlɒn, ən/; Greek: Τάλων, Tálōn) — was a giant automaton
/// made of bronze to protect Europa in Crete from pirates and invaders. He
/// circled the island's shores three times daily.
/// Creates builder traits
#[macro_export]
macro_rules! talus {
  (
    $vis: vis,
    $trt: ident;
    $load: ident, $updt: ident - $ty: ty
  ) => {
      $vis trait $trt {
        fn new_wrap() -> Self;

        fn $load(input: $ty) -> anyResult<Self>
        where
          Self: Sized,
        {
          let mut novo = Self::new_wrap();

          novo.$updt(input)?;

          Ok(novo)
        }

        fn $updt(
          &mut self,
          flines: $ty,
        ) -> anyResult<()>;
      }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Daedalus
/// In Greek mythology, Daedalus (UK: /ˈdiːdələs/, US: /ˈdɛdələs/; Greek:
/// Δαίδαλος; Latin: Daedalus; Etruscan: Taitale) was a skillful architect and
/// craftsman, seen as a symbol of wisdom, knowledge and power. He is the father
/// of Icarus, the uncle of Perdix, and possibly also the father of Iapyx.
/// Parse interactive methods
#[macro_export]
macro_rules! daedalus {
  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_owned', 'get_ref' & 'update'
    (
      $vis: vis,
      $stt: ident;
      $(
        $fld: tt;
        $fown: ident, $fref: ident, $fnup: ident - $town: ty, $tref: ty
      )*
    ) => {
      impl $stt {
        $(
          daedalus!(getown |> $vis, $fld; $fown - $town);
          daedalus!(getref |> $vis, $fld; $fref - $tref);
          daedalus!(update |> $vis, $fld; $fnup - $town);
        )*
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'get' & 'update'
    (
      $vis: vis,
      $stt: ident;
      $(
        $fld: tt;
        $fint: ident, $fnup: ident - $ty: ty
      )*
    ) => {
      impl $stt {
        $(
          daedalus!(getint |> $vis, $fld; $fint - $ty);
          daedalus!(update |> $vis, $fld; $fnup - $ty);
        )*
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement bare 'update', 'increase' & 'decrease'
    (
      $vis: vis,
      $stt: ident;
      $(
        $fld: tt;
        $fnup: ident, $finc: ident, $fdec: ident - $ty: ty
      )*
    ) => {
      impl $stt {
        $(
          daedalus!(update |> $vis, $fld; $fnup - $ty);
          daedalus!(inc |> $vis, $fld; $finc, $fnup - $ty);
          daedalus!(dec |> $vis, $fld; $fdec, $fnup - $ty);
        )*
      }
    };

  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_own'
    (
      getown |> $vis: vis, $fld: tt; $fown: ident - $town: ty
    ) => {
      $vis fn $fown(&self) -> anyResult<$town> {
        Ok(self.$fld.to_owned())
      }
    };

  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_ref'
    (
      getref |> $vis: vis, $fld: tt; $fref: ident - $tref: ty
    ) => {
      $vis fn $fref(&self) -> anyResult<$tref> {
        Ok(&self.$fld)
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'get'
    (
      getint |> $vis: vis, $fld: tt; $fint: ident - $tint: ty
    ) => {
      $vis fn $fint(&self) -> anyResult<$tint> {
        Ok(self.$fld)
      }
    };

  // declare implementation for type
  // interactive methods
  // implement 'update'
    (
      update |> $vis: vis, $fld: tt; $fnup: ident - $tyup: ty
    ) => {
      $vis fn $fnup(&mut self, up: $tyup) -> anyResult<()> {
        self.$fld = up.into();
        Ok(())
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'inc'
    (
      inc |> $vis: vis, $fld: tt; $finc: ident, $fup: ident - $tinc: ty
    ) => {
      $vis fn $finc(&mut self) -> anyResult<()> {
        self.$fup(self.$fld + 1)?;
        Ok(())
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'dec'
    (
      dec |> $vis: vis, $fld: tt; $fdec: ident, $fup: ident - $tdec: ty
    ) => {
      $vis fn $fdec(&mut self) -> anyResult<()> {
        self.$fup(self.$fld - 1)?;
        Ok(())
      }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
