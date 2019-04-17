/// See https://bheisler.github.io/criterion.rs/book/getting_started.html to add more benchmarks.
#[macro_use]
extern crate criterion;

use accumulator::group::{ClassGroup, ElemFrom, Group, UnknownOrderGroup};
use criterion::Criterion;
use rug::Integer;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
  let left = ClassGroup::elem((
    Integer::from_str("16").unwrap(),
    Integer::from_str("9").unwrap(),
    Integer::from_str(
      "47837607866886756167333839869251273774207619337757918597995294777816250058331116325341018110\
      672047217112377476473502060121352842575308793237621563947157630098485131517401073775191194319\
      531549483898334742144138601661120476425524333273122132151927833887323969998955713328783526854\
      198871332313399489386997681827578317938792170918711794684859311697439726596656501594138449739\
      494228617068329664776714484742276158090583495714649193839084110987149118615158361352488488402\
      038894799695420483272708933239751363849397287571692736881031223140446926522431859701738994562\
      9057462766047140854869124473221137588347335081555186814036",
    )
    .unwrap(),
  ));
  let right = left.clone();

  // Generator element.
  let base = ClassGroup::elem((
    Integer::from(2),
    Integer::from(1),
    Integer::from_str(
      "38270086293509404933867071895401019019366095470206334878396235822253000046664893060272814488\
      537637773689901981178801648097082274060247034590097251157726104078788105213920859020152955455\
      625239587118667793715310881328896381140419466618497705721542267109859175999164570663026821483\
      359097065850719591509598145462062654351033736734969435747887449357951781277325201275310759791\
      595382893654663731821371587793820926472466796571719355071267288789719294892126689081990790721\
      631115839756336386618167146591801091079517830057354189504824978512357541217945487761391195650\
      32459702128377126838952995785769100706778680652441494512278",
    )
    .unwrap(),
  ));
  let exp = Integer::from_str("65315").unwrap();
  let g_inv = base.clone();
  let g_sq = ClassGroup::unknown_order_elem();

  let aa = Integer::from_str("16").unwrap();
  let bb = Integer::from_str("105").unwrap();
  let cc = Integer::from_str(
    "4783760786688675616733383986925127377420761933775791859799529477781625005833111632534101811067\
    20472171123774764735020601213528425753087932376215639471576300984851315174010737751911943195315\
    49483898334742144138601661120476425524333273122132151927833887323969998955713328783526854198871\
    33231339948938699768182757831793879217091871179468485931169743972659665650159413844973949422861\
    70683296647767144847422761580905834957146491938390841109871491186151583613524884884020388947996\
    95420483272708933239751363849397287571692736881031223140446926522431859701738994562905746276604\
    7140854869124473221137588347335081555186814207",
  )
  .unwrap();

  // Element which requires one iteration to reduce, represented as a tuple here, since only
  // reduced representations of ClassElem are allowed.
  let g_red = (cc.clone(), bb.clone(), aa.clone());
  let g_norm = (aa.clone(), bb.clone(), cc.clone());

  c.bench_function("group_class_op", move |b| {
    b.iter(|| ClassGroup::op(&left, &right))
  });
  c.bench_function("group_class_exp", move |b| {
    b.iter(|| ClassGroup::exp(&base, &exp))
  });
  c.bench_function("group_class_inv", move |b| {
    b.iter(|| ClassGroup::inv(&g_inv))
  });
  c.bench_function("group_class_normalize", move |b| {
    b.iter_with_setup(
      || g_norm.clone(),
      |g| ClassGroup::normalize(g.0, g.1, g.2),
    )
  });
  c.bench_function("group_class_reduce", move |b| {
    b.iter_with_setup(
      || g_red.clone(),
      |g| ClassGroup::reduce(g.0, g.1, g.2),
    )
  });
  c.bench_function("group_class_square", move |b| {
    b.iter_with_setup(|| g_sq.clone(), |g| ClassGroup::square(&g))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);